import {
  createForm,
  Field,
  FieldValues,
  Form,
  zodForm,
} from "@modular-forms/solid";
import { A } from "@solidjs/router";
import axios, { AxiosError, isAxiosError } from "axios";
import { createSignal } from "solid-js";
import { z } from "zod";
import TextInput from "../components/ui/TextInput";
import { postRequest } from "../lib/httpUtil";
import { UserSession } from "../types";

type TResponse = {
  status: number;
  error_message: string;
  user_session: UserSession;
};
const loginSchema = z.object({
  username: z
    .string()
    .min(8, "Username must contain 8 characters")
    .max(16, "Username must contain less than 16 characters"),
  password: z.string().min(8, "Password must contain 8 characters"),
});

const Login = () => {
  const loginForm = createForm<z.infer<typeof loginSchema>>({
    validate: zodForm(loginSchema),
  });
  const [formError, setFormError] = createSignal<string>();

  const handleSubmit = async (values: FieldValues, event: Event) => {
    event.preventDefault();
    console.log(values, " submit");
  };

  return (
    <div class="flex justify-center h-screen bg-screen text-gray-400">
      <div class="my-auto p-4 w-96 rounded-lg bg-black bg-opacity-20 py-6">
        <h1 class="text-2xl my-6 text-center font-bold">Login</h1>
        <Form of={loginForm} class="py-2" onSubmit={handleSubmit}>
          <Field of={loginForm} name="username">
            {(field) => {
              return (
                <TextInput
                  {...field.props}
                  containerClass="flex flex-col mx-auto w-48"
                  type="text"
                  label="Username"
                  error={field.error}
                  value={field.value}
                  required={true}
                />
              );
            }}
          </Field>
          <Field of={loginForm} name="password">
            {(field) => {
              return (
                <TextInput
                  {...field.props}
                  containerClass="flex flex-col mx-auto w-48"
                  type="password"
                  label="Password"
                  error={field.error}
                  value={field.value}
                  required={true}
                />
              );
            }}
          </Field>
          <div class="flex flex-col mx-auto w-48 mt-4">
            <button
              class="bg-slate-600 rounded-md"
              onClick={() => setFormError("")}
            >
              Login
            </button>
            <div class="text-red-700 text-center w-full my-4 mx-auto break-words">
              {formError()}
            </div>
            <p class=" text-xs mb-1 mx-auto">Don't have an account?</p>
            <button type="button" class="bg-slate-600 rounded-md">
              <A href="/register">Register</A>
            </button>
          </div>
        </Form>
      </div>
    </div>
  );
};

export default Login;
