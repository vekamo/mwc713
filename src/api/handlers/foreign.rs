use gotham::handler::{HandlerFuture };
use gotham::state::{FromState, State};
use hyper::body::Chunk;
use hyper::{Body, Response, StatusCode};
use crate::api::router::{trace_create_response, trace_state_and_body, WalletContainer};
use common::Result;
use wallet::types::{BlockFees, Slate};

pub fn receive_tx(state: State) -> Box<HandlerFuture> {
    Box::new(super::executor::RunHandlerInThread::new(state, handle_receive_tx ) )
}

fn handle_receive_tx(state: &State, body: &Chunk) -> Result<Response<Body>> {
    trace_state_and_body(state, body);
    let mut slate = Slate::deserialize_upgrade(&String::from_utf8(body.to_vec())?)?;
    let wallet = WalletContainer::borrow_from(&state).lock()?;
    wallet.process_sender_initiated_slate(None, &mut slate, None, None)?;
    Ok(trace_create_response(
        &state,
        StatusCode::OK,
        mime::APPLICATION_JSON,
        serde_json::to_string(&slate)?,
    ))
}

pub fn build_coinbase(state: State) -> Box<HandlerFuture> {
    Box::new(super::executor::RunHandlerInThread::new(state, handle_build_coinbase ) )
}

fn handle_build_coinbase(state: &State, body: &Chunk) -> Result<Response<Body>> {
    trace_state_and_body(state, body);
    let block_fees: BlockFees = serde_json::from_slice(&body)?;
    let wallet = WalletContainer::borrow_from(&state).lock()?;
    let cb_data = wallet.build_coinbase(&block_fees)?;
    Ok(trace_create_response(
        &state,
        StatusCode::OK,
        mime::APPLICATION_JSON,
        serde_json::to_string(&cb_data)?,
    ))
}

pub fn receive_invoice(state: State) -> Box<HandlerFuture> {
    Box::new(super::executor::RunHandlerInThread::new(state, handle_receive_invoice ) )
}

fn handle_receive_invoice(state: &State, body: &Chunk) -> Result<Response<Body>> {
    trace_state_and_body(state, body);
    let mut slate: Slate = serde_json::from_slice(&body)?;
    let wallet = WalletContainer::borrow_from(&state).lock()?;
    wallet.process_receiver_initiated_slate(&mut slate)?;
    Ok(trace_create_response(
        &state,
        StatusCode::OK,
        mime::APPLICATION_JSON,
        serde_json::to_string(&slate)?,
    ))
}
