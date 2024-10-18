// login.jsx
import { StytchLogin } from "@stytch/react";
const Login = () => {
  const config = {
    products: ["emailMagicLinks"],
    emailMagicLinksOptions: {
      loginRedirectURL: "http://localhost:5173/authenticate", // TODO: Replace with environment variables
      loginExpirationMinutes: 30,
      signupRedirectURL: "http://localhost:5173/authenticate", // TODO: Replace with environment variables
      signupExpirationMinutes: 30,
    },
  };
  const styles = {
    container: {
      backgroundColor: "#FFFFFF",
      borderColor: "#ADBCC5",
      borderRadius: "8px",
      width: "400px",
    },
    colors: {
      primary: "#19303D",
      secondary: "#5C727D",
      success: "#0C5A56",
      error: "#8B1214",
    },
    buttons: {
      primary: {
        backgroundColor: "#19303D",
        textColor: "#FFFFFF",
        borderColor: "#19303D",
        borderRadius: "4px",
      },
      secondary: {
        backgroundColor: "#FFFFFF",
        textColor: "#19303D",
        borderColor: "#19303D",
        borderRadius: "4px",
      },
    },
    inputs: {
      backgroundColor: "#FFFFFF00",
      borderColor: "#19303D",
      borderRadius: "4px",
      placeholderColor: "#8296A1",
      textColor: "#19303D",
    },
    fontFamily: "Monaco",
    hideHeaderText: false,
    logo: {
      logoImageUrl: "",
    },
  };

  return <StytchLogin config={config} styles={styles} />;
};

export default Login;
