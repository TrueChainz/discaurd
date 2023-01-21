import { useState } from "react";
import { useForm, Controller, SubmitHandler } from "react-hook-form";
import { signIn, useSession } from "next-auth/react";
import TextInput from "../../components/TextInput";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { useRouter } from "next/dist/client/router";
import { register, TRegisterUser } from "../../lib/services";

const FormSchema = z.object({
  username: z.string().min(8, "Username must contain at least 8 character(s)"),
  email: z
    .string()
    .min(8, "email must contain at least 6 character(s)")
    .email(),
  password: z.string().min(8, "Password must contain at least 8 character(s)"),
});

type FormSchemaType = z.infer<typeof FormSchema>;

function Register() {
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
    try {
      const registerRespone = await register(data as TRegisterUser);
      const signInResponse = await signIn("credentials", {
        redirect: false,
        username: registerRespone.username,
        password: data.password,
      });

      if (signInResponse.ok) {
        return router.push("/");
      }
      setFormError(signInResponse.error);
    } catch (err) {
      setFormError(err.message);
    }
  };

  return (
    <div className="bg-screen flex h-screen justify-center text-gray-400">
      <div className="my-auto w-96 rounded-lg bg-black bg-opacity-20 p-4 py-6 outline outline-base-300">
        <h1 className="my-6 text-center text-2xl font-bold">Register</h1>
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
            name="email"
            control={control}
            render={({ field }) => {
              return (
                <TextInput
                  {...field}
                  containerClass="flex flex-col mx-auto w-48 "
                  type="email"
                  label="Email"
                  error={errors.email?.message}
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
              className=" btn-accent btn-sm btn mx-auto w-1/2 rounded-md"
              type="submit"
              onClick={() => setFormError("")}
            >
              Register
            </button>
            <div className="min-h-6 my-2 mx-auto w-full break-words text-center text-red-700">
              {formError}
            </div>
            <p className="my-1 mx-auto text-xs">Already have an account?</p>
            <button
              type="button"
              className="btn-outline btn-accent btn-sm btn mx-auto w-1/2 rounded-md bg-base-300 "
              onClick={() => signIn()}
            >
              Login
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

export default Register;
