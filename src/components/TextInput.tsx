import { cva } from "class-variance-authority";

type TextInputProps = {
  ref: (element: HTMLInputElement) => void;
  type: "text" | "email" | "tel" | "password" | "url" | "number" | "date";
  name: string;
  value: string | number | undefined;
  onChange: any;
  onBlur: any;
  placeholder?: string;
  required?: boolean;
  label?: string;
  error?: string;

  containerClass: string;
};

const cvaInput = cva("input-accent input h-8 rounded-md  px-2", {
  variants: {
    isError: {
      true: "input-error",
    },
  },
});

const TextInput = (props: TextInputProps) => {
  return (
    <div className={props.containerClass}>
      {props.label && (
        <label htmlFor={props.name} className="label-text label text-sm">
          {props.label} {props.required && <span>*</span>}
        </label>
      )}
      <input
        className={cvaInput({ isError: !!props.error })}
        // ref={props.ref}
        id={props.name}
        name={props.name}
        value={props.value || ""}
        aria-invalid={!!props.error}
        aria-errormessage={`${props.name}-error`}
        onBlur={props.onBlur}
        onChange={props.onChange}
        type={props.type}
      />
      <div className="mx-auto mt-1 text-center">
        <div
          style={{ minHeight: "1rem" }}
          className="mx-auto w-48 break-words text-center text-sm leading-3 text-red-700"
        >
          {props.error}
        </div>
      </div>
    </div>
  );
};

export default TextInput;
