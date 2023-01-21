import { Dispatch, SetStateAction, useEffect, useState } from "react";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { SubmitHandler, useForm } from "react-hook-form";
import { sendFriendRequest } from "../lib/services";

interface Props {
  setModalActive: Dispatch<SetStateAction<boolean>>;
  username: string;
}

const FormSchema = z.object({
  target_username: z
    .string()
    .min(8, "Username must contain at least 8 character(s)"),
});

type FormSchemaType = z.infer<typeof FormSchema>;

const AddFriendModal = ({ setModalActive, username }: Props) => {
  const [requestError, setRequestError] = useState("");
  const {
    handleSubmit,
    formState: { errors, isSubmitSuccessful },
    register,
    getValues,
  } = useForm<FormSchemaType>({
    resolver: zodResolver(FormSchema),
  });

  const resetError = () => {
    if (requestError) {
      setRequestError("");
    }
  };

  const onSubmit: SubmitHandler<FormSchemaType> = async (data, e) => {
    e.preventDefault();
    resetError();
    try {
      const payload = {
        source_username: username,
        target_username: data.target_username,
      };
      const response = await sendFriendRequest(payload);
      if (response.success) {
        return true;
      }
      setRequestError(response.error_message);
    } catch (err) {
      setRequestError(err.message);
    }
    throw "";
  };

  return (
    <div className="modal pointer-events-auto visible opacity-100">
      <form
        className="modal-box flex flex-col items-center"
        onSubmit={handleSubmit(onSubmit)}
      >
        <h3 className="mb-6 text-xl font-bold">Add Friend</h3>
        <input
          name="target_username"
          {...register("target_username")}
          placeholder="Type here"
          onChange={() => resetError()}
          className={`${
            errors.target_username || requestError
              ? "input-error"
              : "input-accent"
          } input-bordered input w-full max-w-xs`}
        />

        {errors.target_username && (
          <div className="mt-4 text-center text-error">
            {errors.target_username.message}
          </div>
        )}
        {requestError && (
          <div className="mt-4 text-center text-error">{requestError}</div>
        )}

        {isSubmitSuccessful && (
          <div className="mt-4 text-success">
            Success! Your friend request to {getValues("target_username")} was
            sent.
          </div>
        )}
        <div className="modal-action">
          <button
            className="btn-outline btn-accent btn-sm btn normal-case"
            type="button"
            onClick={() => setModalActive(false)}
          >
            Back
          </button>

          <button className=" btn-accent btn-sm btn font-bold normal-case">
            Send Friend Request
          </button>
        </div>
      </form>
    </div>
  );
};

export default AddFriendModal;
