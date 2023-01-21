import { axiosInstance } from "./httpUtil";

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
    if (response.success == false)
      throw new Error("Unexpected error! Please try again later.");

    return response.user;
  } catch (error) {
    console.log(error);
    if (!error.response?.data) {
      throw new Error("Unexpected error! Please try again later.");
    }
    throw new Error(error.response.data.error_message);
  }
}

export interface TRegisterUser {
  username: string;
  password: string;
  email: string;
}

export async function register(data: TRegisterUser): Promise<TUser> {
  try {
    let response = await axiosInstance
      .post("http://127.0.0.1:3000/api/user/register", {
        email: data.email,
        username: data.username,
        password: data.password,
      })
      .then((response) => {
        return response.data;
      });

    console.log("REGISTERING USER", response);
    if (response.success == false)
      throw new Error("Unexpected error! Please try again later.");

    return response.user;
  } catch (error) {
    console.log(error);
    if (!error.response?.data) {
      throw new Error("Unexpected error! Please try again later.");
    }
    console.log(error.response.data.error_message);
    throw new Error(error.response.data.error_message);
  }
}

export interface TFriendRequest {
  source_username: string;
  target_username: string;
}

export interface TGenericResponse {
  success: boolean;
  error_message: string;
}

export async function sendFriendRequest(
  data: TFriendRequest
): Promise<TGenericResponse> {
  try {
    let response = await axiosInstance
      .post<TGenericResponse>("http://127.0.0.1:3000/api/friend/add", {
        source_username: data.source_username,
        target_username: data.target_username,
      })
      .then((response) => {
        console.log(response);
        return response.data;
      });

    console.log("Sending Friend Request", response);
    if (response.success == false)
      throw new Error("Unexpected error! Please try again later.");

    return response;
  } catch (error) {
    if (!error.response?.data?.error_message) {
      throw new Error("Unexpected error! Please try again later.");
    }
    console.log(error.response);

    throw new Error(error.response.data.error_message);
  }
}
