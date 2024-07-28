class Fetch {
  private static baseURL: string = process.env.API_BASE_URL || "http://nginx"; // ベースURLを設定

  private static instance: Fetch; // Fetchのシングルトンインスタンスを保持

  // Fetchクラスのインスタンスを取得するメソッド
  public static getInstance(): Fetch {
    if (!Fetch.instance) {
      Fetch.instance = new Fetch();
    }
    return Fetch.instance;
  }

  // APIエンドポイントにリクエストを送信するメソッド
  public async fetch<T>(endpoint: string, options?: RequestInit): Promise<T> {
    try {
      // fetchメソッドを使用してリクエストを送信し、レスポンスを取得
      const response = await fetch(`${Fetch.baseURL}${endpoint}`, { cache: "no-cache", ...options });

      // レスポンスが正常でない場合、エラーメッセージを投げる
      if (!response.ok) {
        throw new Error(`Fetch request failed with status ${response.status}`);
      }

      // レスポンスをJSON形式で返す
      return await response.json();
    } catch (error) {
      // エラーメッセージをコンソールに出力し、再度投げる
      console.error("Fetch error:", error);
      throw error;
    }
  }
}

export default Fetch;
