import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useForm, Controller, SubmitHandler } from "react-hook-form";
import { signIn, useSession } from "next-auth/react";
import TextInput from "../../components/TextInput";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { useRouter } from "next/dist/client/router";

const FormSchema = z.object({
  username: z.string().min(8, "Username must contain at least 8 character(s)"),
  password: z.string().min(8, "Password must contain at least 8 character(s)"),
});

type FormSchemaType = z.infer<typeof FormSchema>;

function Login() {
  const [formError, setFormError] = useState("");
  const {
    control,
    handleSubmit,
    formState: { errors },
  } = useForm<FormSchemaType>({
    resolver: zodResolver(FormSchema),
  });

  const { data: session, status } = useSession();
  const router = useRouter();

  if (status === "authenticated") {
    router.push("/");
  }

  const onSubmit: SubmitHandler<FormSchemaType> = async (data, e) => {
    e.preventDefault();
    const response = await signIn("credentials", {
      redirect: false,
      ...data,
    });
    if (response.ok) {
      return router.push("/");
    }

    setFormError(response.error);
  };

  return (
    <div className="bg-screen flex h-screen justify-center text-gray-400">
      <div className="my-auto w-96 rounded-lg bg-black bg-opacity-20 p-4 py-6 outline outline-base-300">
        <h1 className="my-6 text-center text-2xl font-bold">Login</h1>
        <form onSubmit={handleSubmit(onSubmit)}>
          <Controller
            name="username"
            control={control}
            render={({ field }) => {
              return (
                <TextInput
                  {...field}
                  containerClass="flex flex-col mx-auto w-48"
                  type="text"
                  label="Username"
                  error={errors.username?.message}
                  value={field.value}
                  required={true}
                />
              );
            }}
          />
          <Controller
            name="password"
            control={control}
            render={({ field }) => {
              return (
                <TextInput
                  {...field}
                  containerClass="flex flex-col mx-auto w-48 "
                  type="password"
                  label="Password"
                  error={errors.password?.message}
                  value={field.value}
                  required={true}
                />
              );
            }}
          />
          <div className="mx-auto mt-4 flex w-48 flex-col">
            <button
              className="btn-accent btn-sm btn mx-auto w-1/2 rounded-md"
              type="submit"
              onClick={() => setFormError("")}
            >
              Login
            </button>
            <div className="min-h-6 my-2 mx-auto w-full break-words text-center text-red-700">
              {formError}
            </div>
            <p className="my-1 mx-auto text-xs">Don't have an account?</p>
            <button
              type="button"
              className="btn-outline btn-accent  btn-sm btn mx-auto w-1/2 rounded-md bg-base-300"
              onClick={() => router.push("/auth/register")}
            >
              Register
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

export default Login;
