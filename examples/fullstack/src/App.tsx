import { createSignal, Show, type Component } from "solid-js";
import RpcClient from "./rpc-client";
import type { GetUser } from "./rpc-client";
import logo from "./logo.svg";
import styles from "./App.module.css";

type User = Awaited<ReturnType<GetUser>>["Ok"];

const client = new RpcClient({
  transport: {
    type: "http",
    host: "localhost",
    port: 8080,
    path: "/rpc",
  },
});

const App: Component = () => {
  const [user, setUser] = createSignal<User | null>(null);
  const [registered, setRegistered] = createSignal<User["id"] | null>(null);

  const onSubmitRegisterUser = (e: Event) => {
    e.preventDefault();
    const name = ((e.target as HTMLFormElement).elements[0] as HTMLInputElement)
      .value;
    const age = parseInt(
      ((e.target as HTMLFormElement).elements[0] as HTMLInputElement).value,
    );
    client.register_user(name, age);
  };

  const onSubmitGetUser = async (e: Event) => {
    e.preventDefault();
    const id = parseInt(
      ((e.target as HTMLFormElement).elements[0] as HTMLInputElement).value,
    );
    const user = await client.get_user(id);
    if (user.Ok) setUser(user.Ok);
  };

  return (
    <div class={styles.App}>
      <header class={styles.header}>
        <img src={logo} class={styles.logo} alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          class={styles.link}
          href="https://github.com/solidjs/solid"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn Solid
        </a>
        <form onSubmit={onSubmitGetUser}>
          <input type="number" placeholder="User ID" />
          <button type="submit">Get</button>
        </form>
        <Show when={user()}>
          <div>
            <p>User ID: {user()?.id}</p>
            <p>Name: {user()?.name}</p>
            <p>Age: {user()?.age}</p>
          </div>
        </Show>
        <form onSubmit={onSubmitRegisterUser}>
          <input type="text" placeholder="Name" />
          <input type="number" placeholder="Age" />
          <button type="submit">Register</button>
          <Show when={registered()}>
            <div>
              <p>User ID: {registered()}</p>
            </div>
          </Show>
        </form>
      </header>
    </div>
  );
};

export default App;
