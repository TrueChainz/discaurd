import {
  createForm,
  FieldValues,
  Form,
  Field,
  zodForm,
} from "@modular-forms/solid";
import { A } from "@solidjs/router";
import { createSignal } from "solid-js";
import { z } from "zod";
import TextInput from "../components/ui/TextInput";

const registerSchema = z.object({
  username: z
    .string()
    .min(8, "Username must contain 8 characters")
    .max(16, "Username must contain less than 16 characters"),
  email: z.string().email("Invalid email format"),
  password: z.string().min(8, "Password must contain 8 characters"),
});

const Register = () => {
  const registerForm = createForm<z.infer<typeof registerSchema>>({
    validate: zodForm(registerSchema),
  });
  const [formError, setFormError] = createSignal<string>();

  const handleSubmit = async (values: FieldValues, event: Event) => {
    event.preventDefault();
    console.log(values, " submit");
  };

  return (
    <div class="flex justify-center h-screen bg-screen text-gray-400">
      <div class="my-auto w-96 rounded-lg bg-black bg-opacity-20 p-4  py-6">
        <h1 class="text-2xl my-6 text-center font-bold">Register</h1>
        <Form of={registerForm} class="py-2" onSubmit={handleSubmit}>
          <Field of={registerForm} name="username">
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
          <Field of={registerForm} name="email">
            {(field) => {
              return (
                <TextInput
                  {...field.props}
                  containerClass="flex flex-col mx-auto w-48"
                  type="email"
                  label="Email"
                  error={field.error}
                  value={field.value}
                  required={true}
                />
              );
            }}
          </Field>
          <Field of={registerForm} name="password">
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
              Register
            </button>
            <span class="text-red-700 text-center w-full my-4 mx-auto break-words">
              {formError()}
            </span>
            <p class="text-xs my-1 mx-auto">Already have an account?</p>
            <button
              type="button"
              class="bg-slate-600 rounded-md"
              // onClick={() => navigate("/login")}
            >
              <A href="/login">Login</A>
            </button>
          </div>
        </Form>
      </div>
    </div>
  );
};

export default Register;
