use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{get, web, HttpResponse};
use web3::contract::Options;
use web3::transports::Http;
use web3::types::{Address, BlockId, U256, U64};
use web3::Web3;

use crate::constants::{ApiErrorMessage, ApiResponse, ApiResponseStatus};
use crate::mongo::models::WalletHistory;
use crate::mongo::Connection;
use crate::tools::erc20_at;

#[get("/archive/wallet_history/{address}/{token}/{block}")]
pub async fn wallet_history(
    path: web::Path<(String, String, i64)>,
    db: web::Data<Connection>,
    ronin: web::Data<Web3<Http>>,
) -> HttpResponse {
    let (address, token, block) = path.into_inner();

    let balance = db.history(address.clone(), token.clone(), block).await;

    if balance.is_some() {
        HttpResponse::Ok()
            .insert_header((
                HeaderName::from_static("x-cache"),
                HeaderValue::from_static("true"),
            ))
            .json(ApiResponse {
                status: ApiResponseStatus::Ok,
                data: Some(balance.unwrap().into()),
                error: None,
            })
    } else {
        match token.parse::<Address>() {
            Ok(token_parsed) => {
                let contract = erc20_at(token_parsed, &ronin).unwrap();

                match address.parse::<Address>() {
                    Ok(address_parsed) => {
                        let result: Result<U256, _> = contract
                            .query(
                                "balanceOf",
                                (address_parsed,),
                                None,
                                Options::default(),
                                BlockId::from(U64::from(block)),
                            )
                            .await;

                        match result {
                            Ok(balance) => {
                                let history = WalletHistory {
                                    token,
                                    block,
                                    address,
                                    balance: balance.as_u64(),
                                };

                                let _ = db.insert(&history).await;

                                HttpResponse::Ok().json(ApiResponse {
                                    status: ApiResponseStatus::Ok,
                                    data: Some(history.into()),
                                    error: None,
                                })
                            }
                            Err(_) => HttpResponse::InternalServerError().json(
                                ApiErrorMessage::create(90500, "Failed fetching balance from rpc."),
                            ),
                        }
                    }
                    Err(_) => HttpResponse::InternalServerError()
                        .json(ApiErrorMessage::create(90502, "Failed parsing address")),
                }
            }
            Err(_) => HttpResponse::InternalServerError().json(ApiErrorMessage::create(
                90501,
                "Failed parsing token address",
            )),
        }
    }
}
