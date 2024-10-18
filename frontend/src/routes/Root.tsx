import { useStytchSession } from "@stytch/react";
import { Outlet, useLocation } from "react-router";

const OPEN_ROUTES = ["/login", "/authenticate"];

const Root = () => {
  const { session } = useStytchSession();
  const location = useLocation();

  if (!session && OPEN_ROUTES.includes(location.pathname)) {
    window.location.href = "/login";
  }

  return (
    <>
      Root: <Outlet />
    </>
  );
};

export default Root;
