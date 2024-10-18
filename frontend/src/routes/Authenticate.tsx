import React, { useEffect } from "react";
import { useStytch, useStytchSession } from "@stytch/react";

const Authenticate = () => {
  const stytch = useStytch();
  const { session } = useStytchSession();
  useEffect(() => {
    if (session) {
      window.location.href = "/"; // Redirect to home page
    } else {
      const token = new URLSearchParams(window.location.search).get("token");
      if (token) {
        stytch.magicLinks.authenticate(token, {
          session_duration_minutes: 3600,
        });
      }
    }
  }, [session, stytch]);

  return <div>Authenticate</div>;
};

export default Authenticate;
