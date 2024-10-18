import "./App.css";
import { StytchProvider } from "@stytch/react";
import { StytchUIClient } from "@stytch/vanilla-js";
import Login from "./routes/Login";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Authenticate from "./routes/Authenticate";
import Root from "./routes/Root";

const stytchOptions = {
  cookieOptions: {
    opaqueTokenCookieName: "stytch_session",
    jwtCookieName: "stytch_session_jwt",
    path: "",
    availableToSubdomains: false,
    domain: "",
  },
};

const stytch = new StytchUIClient(
  import.meta.env.VITE_STYTCH_TOKEN,
  stytchOptions
);

const router = createBrowserRouter([
  {
    path: "/",
    element: <Root />,
    children: [
      {
        path: "/login",
        element: <Login />,
      },
      { path: "/authenticate", element: <Authenticate /> },
    ],
  },
]);

function App() {
  return (
    <StytchProvider stytch={stytch}>
      <RouterProvider router={router} />
    </StytchProvider>
  );
}

export default App;
