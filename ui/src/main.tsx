import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { App } from "./App";
import "./App.css";

const raiz = document.getElementById("root");
if (!raiz) {
  throw new Error("No está el contenedor de la aplicación.");
}

createRoot(raiz).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
