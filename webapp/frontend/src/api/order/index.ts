import Axios from "../axios"; // Axiosインスタンスをインポート
import Fetch from "../fetch"; // Fetchインスタンスをインポート
import queryString from "query-string"; // クエリパラメータを文字列に変換するためのライブラリをインポート

// Order型を定義。注文の各フィールドが何を意味するかを説明
export type Order = {
  id: number; // 注文ID
  status: string; // 注文のステータス（例: "pending", "completed"）
  node_id: number; // ノードID（注文の位置を示す）
  area_id: number; // エリアID（注文が属するエリア）
  tow_truck_id: number; // レッカー車ID
  car_value: number; // 車の価値
  client_id: number; // クライアントID
  client_username: string; // クライアントのユーザー名
  dispatcher_user_id: number; // ディスパッチャーユーザーID
  dispatcher_username: string; // ディスパッチャーのユーザー名
  driver_user_id: number; // ドライバーユーザーID
  driver_username: string; // ドライバーのユーザー名
  order_time: string; // 注文時間
  completed_time: string; // 完了時間
};

// OrdersQueryParams型を定義。クエリパラメータのフィールドを説明
export type OrdersQueryParams = {
  status: string; // 注文のステータス（例: "pending", "completed"）
  sort_by: string; // ソートの基準となるフィールド（例: "order_time", "car_value"）
  sort_order: string; // ソートの順序（例: "asc", "desc"）
};

// AxiosとFetchのインスタンスを作成
const AxiosInstance = Axios.getInstance(); // Axiosインスタンスを取得
const FetchInstance = Fetch.getInstance(); // Fetchインスタンスを取得

// 注文リストを取得する関数
export const fetchOrders = async (query_params: OrdersQueryParams, area: number | null, session_token: string) => {
  // クエリパラメータとエリアをログに出力
  console.log('Sending query params:', query_params, 'area:', area);

  // クエリパラメータを文字列に変換
  const queryParams = queryString.stringify({
    ...query_params, // クエリパラメータを展開
    status: "pending", // ステータスを "pending" に設定
    sort_by: "order_time", // ソート基準を "order_time" に設定
    sort_order: "asc", // ソート順序を "asc" に設定
    area // エリアを設定
  });

  // 生成されたクエリ文字列をログに出力
  console.log('Generated query string:', queryParams);

  // Fetchインスタンスを使って注文リストを取得
  const orders = await FetchInstance.fetch<Order[]>(`/api/order/list?${queryParams}`, {
    headers: { Authorization: session_token } // セッショントークンをヘッダーに設定
  });

  // 注文リストを返す
  return orders;
};

// 特定の注文を取得する関数
export const fetchOrder = async (order_id: string, session_token: string) => {
  // Fetchインスタンスを使って特定の注文を取得
  const order = await FetchInstance.fetch<Order>(`/api/order/${order_id}`, {
    headers: { Authorization: session_token } // セッショントークンをヘッダーに設定
  });

  // 注文を返す
  return order;
};

// レッカー車を手配する関数
export const arrangeTowTruck = async (
  dispatcher_id: number, // ディスパッチャーID
  order_id: number, // 注文ID
  tow_truck_id: number, // レッカー車ID
  order_time: string, // 注文時間
  session_token: string // セッショントークン
) => {
  // Axiosインスタンスを使ってレッカー車を手配
  await AxiosInstance.post(
    "/api/order/dispatcher", // APIエンドポイント
    {
      dispatcher_id, // ディスパッチャーID
      order_id, // 注文ID
      tow_truck_id, // レッカー車ID
      order_time // 注文時間
    },
    { timeout: 5000, headers: { Authorization: session_token } } // タイムアウトとヘッダーを設定
  );
};
