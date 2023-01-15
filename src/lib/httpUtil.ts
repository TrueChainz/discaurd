import axios from "axios";

const axiosInstance = axios.create({ baseURL: "http://127.0.0.1:3000/api/" });

interface TCredentials {
  username: string;
  password: string;
}

interface TUser {
  id: string;
  username: string;
}

export async function authenticate(credentials: TCredentials): Promise<TUser> {
  try {
    let response = await axiosInstance
      .post("/user/login", {
        username: credentials.username,
        password: credentials.password,
      })
      .then((response) => {
        console.log(response?.data);

        return response.data;
      });
    console.log("AUTHENTICATING USER", response);
    if (response.status != "201")
      throw new Error("Unexpected error! Please try again later.");

    return response.user;
  } catch (error) {
    console.log(error);
    if (!error.response) {
      throw new Error("Unexpected error! Please try again later.");
    }
    throw new Error(error.response.data.error_message);
  }
  return null;
}
