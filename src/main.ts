import "./app.css";
import App from "./App.svelte";
import { mount } from "svelte";

document.addEventListener("contextmenu", (e) => e.preventDefault()); // Stop right clicks

const app = mount(App, { target: document.getElementById("app")! });

export default app;
