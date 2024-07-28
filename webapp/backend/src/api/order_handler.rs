use crate::domains::dto::order::{
    ClientOrderRequestDto, DispatcherOrderRequestDto, UpdateOrderStatusRequestDto,
}; // 各種注文リクエスト用のデータ転送オブジェクト（DTO）をインポート
use crate::domains::order_service::OrderService; // OrderServiceをインポート
use crate::errors::AppError; // アプリケーション固有のエラー型をインポート
use crate::repositories::auth_repository::AuthRepositoryImpl; // 認証リポジトリの実装をインポート
use crate::repositories::map_repository::MapRepositoryImpl; // 地図リポジトリの実装をインポート
use crate::repositories::order_repository::OrderRepositoryImpl; // 注文リポジトリの実装をインポート
use crate::repositories::tow_truck_repository::TowTruckRepositoryImpl; // レッカー車リポジトリの実装をインポート
use actix_web::{web, HttpResponse}; // Actix-webのwebモジュールとHttpResponseをインポート
use serde::Deserialize; // シリアライズ/デシリアライズをサポートするためのSerdeクレートをインポート

// 注文のステータスを更新するハンドラー
pub async fn update_order_status_handler(
    service: web::Data<
        OrderService<
            OrderRepositoryImpl,
            TowTruckRepositoryImpl,
            AuthRepositoryImpl,
            MapRepositoryImpl,
        >,
    >, // 注文サービスのインスタンスを依存性として受け取る
    req: web::Json<UpdateOrderStatusRequestDto>, // JSONリクエストボディとして受け取る
) -> Result<HttpResponse, AppError> { // 戻り値はHttpResponseかAppError
    match service.update_order_status(req.order_id, &req.status).await { // サービスを使って注文ステータスを更新
        Ok(_) => Ok(HttpResponse::Ok().finish()), // 成功したら200 OKを返す
        Err(err) => Err(err), // 失敗したらエラーを返す
    }
}

// 注文を取得するハンドラー
pub async fn get_order_handler(
    service: web::Data<
        OrderService<
            OrderRepositoryImpl,
            TowTruckRepositoryImpl,
            AuthRepositoryImpl,
            MapRepositoryImpl,
        >,
    >, // 注文サービスのインスタンスを依存性として受け取る
    path: web::Path<i32>, // URLパスパラメータを受け取る
) -> Result<HttpResponse, AppError> { // 戻り値はHttpResponseかAppError
    match service.get_order_by_id(path.into_inner()).await { // サービスを使って注文をIDで取得
        Ok(order) => Ok(HttpResponse::Ok().json(order)), // 成功したら200 OKと注文データを返す
        Err(err) => Err(err), // 失敗したらエラーを返す
    }
}

// ページネーションされた注文リストを取得するためのクエリパラメータを定義
#[derive(Deserialize, Debug)]
pub struct PaginatedOrderQuery {
    page: Option<i32>,
    page_size: Option<i32>,
    sort_by: Option<String>,
    sort_order: Option<String>,
    status: Option<String>,
    area: Option<i32>,
}

// ページネーションされた注文リストを取得するハンドラー
pub async fn get_paginated_orders_handler(
    service: web::Data<
        OrderService<
            OrderRepositoryImpl,
            TowTruckRepositoryImpl,
            AuthRepositoryImpl,
            MapRepositoryImpl,
        >,
    >, // 注文サービスのインスタンスを依存性として受け取る
    query: web::Query<PaginatedOrderQuery>, // クエリパラメータを受け取る
) -> Result<HttpResponse, AppError> { // 戻り値はHttpResponseかAppError
    // クエリパラメータをログに出力
    println!("Received query params: {:?}", query);

    match service
        .get_paginated_orders(
            query.page.unwrap_or(0),
            query.page_size.unwrap_or(10),
            query.sort_by.clone(),
            query.sort_order.clone(),
            query.status.clone(),
            query.area,
        )
        .await
    {
        Ok(orders) => Ok(HttpResponse::Ok().json(orders)), // 成功したら200 OKと注文リストを返す
        Err(err) => {
            // エラーをログに出力
            println!("Error occurred: {:?}", err);
            Err(err) // 失敗したらエラーを返す
        },
    }
}

// クライアントからの注文を作成するハンドラー
pub async fn create_client_order_handler(
    service: web::Data<
        OrderService<
            OrderRepositoryImpl,
            TowTruckRepositoryImpl,
            AuthRepositoryImpl,
            MapRepositoryImpl,
        >,
    >, // 注文サービスのインスタンスを依存性として受け取る
    req: web::Json<ClientOrderRequestDto>, // JSONリクエストボディとして受け取る
) -> Result<HttpResponse, AppError> { // 戻り値はHttpResponseかAppError
    match service
        .create_client_order(req.client_id, req.node_id, req.car_value)
        .await
    {
        Ok(_) => Ok(HttpResponse::Created().finish()), // 成功したら201 Createdを返す
        Err(err) => Err(err), // 失敗したらエラーを返す
    }
}

// ディスパッチャーからの注文を作成するハンドラー
pub async fn create_dispatcher_order_handler(
    service: web::Data<
        OrderService<
            OrderRepositoryImpl,
            TowTruckRepositoryImpl,
            AuthRepositoryImpl,
            MapRepositoryImpl,
        >,
    >, // 注文サービスのインスタンスを依存性として受け取る
    req: web::Json<DispatcherOrderRequestDto>, // JSONリクエストボディとして受け取る
) -> Result<HttpResponse, AppError> { // 戻り値はHttpResponseかAppError
    match service
        .create_dispatcher_order(
            req.order_id,
            req.dispatcher_id,
            req.tow_truck_id,
            req.order_time,
        )
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()), // 成功したら200 OKを返す
        Err(err) => Err(err), // 失敗したらエラーを返す
    }
}