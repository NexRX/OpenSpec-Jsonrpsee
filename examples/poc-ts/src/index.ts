// JSON RPC 2.0 Types
interface JsonRpcRequest<T> {
  jsonrpc: "2.0";
  method: string;
  params?: T;
  id: string | number;
}

interface JsonRpcResponse<T = any> {
  jsonrpc: "2.0";
  result?: T;
  error?: JsonRpcError;
  id: string | number | null;
}

interface JsonRpcError {
  code: number;
  message: string;
  data?: any;
}
/**
 * Generic RPC Error class for method not found, invalid params, etc.
 */
class RpcError extends Error {
  public code: number;
  public data?: any;

  constructor(error: JsonRpcError) {
    super(error.message);
    this.name = "RpcError";
    this.code = error.code;
    this.data = error.data;
  }
}

// JSON RPC Client Class
class JsonRpcClient {
  private url: string;
  private headers: Record<string, string>;
  private requestId: number = 1;

  constructor(url: string, headers: Record<string, string> = {}) {
    this.url = url;
    this.headers = {
      "Content-Type": "application/json",
      ...headers,
    };
  }

  // Make a single RPC call
  protected async call<R = any, P = any>(
    method: string,
    params: P,
  ): Promise<R> {
    const request: JsonRpcRequest<P> = {
      jsonrpc: "2.0",
      method,
      params,
      id: this.requestId++,
    };

    try {
      const response = await fetch(this.url, {
        method: "POST",
        headers: this.headers,
        body: JSON.stringify(request),
      });

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`);
      }

      const jsonResponse: JsonRpcResponse<R> = await response.json();

      if (jsonResponse.error) {
        throw new JsonRpcClientError(
          jsonResponse.error.message,
          jsonResponse.error.code,
          jsonResponse.error.data,
        );
      }

      return jsonResponse.result as R;
    } catch (error) {
      if (error instanceof JsonRpcClientError) {
        throw error;
      }
      throw new RpcError({
        code: -32603,
        message: `JSON RPC call failed: ${error instanceof Error ? error.message : String(error)}`,
        data: error,
      });
    }
  }

  // Make a notification call (no response expected)
  protected async notify(
    method: string,
    params?: any[] | Record<string, any>,
  ): Promise<void> {
    const request = {
      jsonrpc: "2.0" as const,
      method,
      params,
      // No 'id' field for notifications
    };

    const response = await fetch(this.url, {
      method: "POST",
      headers: this.headers,
      body: JSON.stringify(request),
    });

    if (!response.ok) {
      throw new RpcError({
        code: -32603,
        message: `HTTP ${response.status}: ${response.statusText}`,
        data: response.body,
      });
    }
  }

  // Make multiple calls in a batch
  protected async batch<R = any, P = any>(
    calls: Array<{ method: string; params: P }>,
  ): Promise<R[]> {
    const requests: JsonRpcRequest<P>[] = calls.map((call) => ({
      jsonrpc: "2.0",
      method: call.method,
      params: call.params,
      id: this.requestId++,
    }));

    const response = await fetch(this.url, {
      method: "POST",
      headers: this.headers,
      body: JSON.stringify(requests),
    });

    if (!response.ok) {
      throw new RpcError({
        code: -32603,
        message: `HTTP ${response.status}: ${response.statusText}`,
        data: response.body,
      });
    }

    const jsonResponses: JsonRpcResponse<R>[] = await response.json();
    const results: R[] = [];

    for (const jsonResponse of jsonResponses) {
      if (jsonResponse.error) {
        throw new JsonRpcClientError(
          jsonResponse.error.message,
          jsonResponse.error.code,
          jsonResponse.error.data,
        );
      }
      results.push(jsonResponse.result as R);
    }

    return results;
  }
}

// Custom error class for JSON RPC errors
class JsonRpcClientError extends Error {
  public code: number;
  public data?: any;

  constructor(message: string, code: number, data?: any) {
    super(message);
    this.name = "JsonRpcClientError";
    this.code = code;
    this.data = data;
  }
}

// Simple function-based approach (alternative to class)
async function jsonRpcCall<R = any, P = any>(
  url: string,
  method: string,
  params: P,
  headers: Record<string, string> = {},
): Promise<R> {
  const request: JsonRpcRequest<P> = {
    jsonrpc: "2.0",
    method,
    params,
    id: Date.now(),
  };

  const response = await fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...headers,
    },
    body: JSON.stringify(request),
  });

  if (!response.ok) {
    throw new RpcError({
      code: -32603,
      message: `HTTP ${response.status}: ${response.statusText}`,
      data: response.body,
    });
  }
  const jsonResponse: JsonRpcResponse<R> = await response.json();
  if (jsonResponse.error) throw new RpcError(jsonResponse.error);
  return jsonResponse.result as R;
}

export { JsonRpcClient, JsonRpcClientError, jsonRpcCall };

// Example usage

// TODO: everything above can be static below needs to be generated templated etc.
// May have issues with types. probably need to get a *set* of all types and then gen and insert upfront then append the class/function generation

type User = {
  id: number;
  name: string;
  age: number;
};

class ExampleClient extends JsonRpcClient {
  constructor(url: string, headers: Record<string, string> = {}) {
    super(url, headers);
  }

  public getUser(id: number): Promise<number> {
    return this.call("get_user", [id]);
  }

  public registerUser(name: string, age: number): Promise<User> {
    return this.call("register_user", [name, age]);
  }
}

const client = new ExampleClient("http://localhost:8080/rpc");
client.registerUser("John Doe", 30).then((user) => {
  console.log("Registered user:", user);

  client.getUser(1).then((user) => {
    console.log("User:", user);
  });
});
