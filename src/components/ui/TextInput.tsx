import { JSX, splitProps } from "solid-js";

type TextInputProps = {
  ref: (element: HTMLInputElement) => void;
  type: "text" | "email" | "tel" | "password" | "url" | "number" | "date";
  name: string;
  value: string | number | undefined;
  onInput: JSX.EventHandler<HTMLInputElement, InputEvent>;
  onChange: JSX.EventHandler<HTMLInputElement, Event>;
  onBlur: JSX.EventHandler<HTMLInputElement, FocusEvent>;
  placeholder?: string;
  required?: boolean;
  label?: string;
  error?: string;

  containerClass: string;
};

const TextInput = (props: TextInputProps) => {
  const [, inputProps] = splitProps(props, [
    "value",
    "label",
    "error",
    "containerClass",
  ]);
  return (
    <div class={props.containerClass}>
      {props.label && (
        <label for={props.name} class="text-sm">
          {props.label} {props.required && <span>*</span>}
        </label>
      )}
      <input
        {...inputProps}
        class="rounded-md bg-slate-200 px-2 h-7"
        id={props.name}
        value={props.value || ""}
        aria-invalid={!!props.error}
        aria-errormessage={`${props.name}-error`}
      />
      <span class="text-red-700 h-7 w-fit mx-auto break-words leading-3 p-1 text-center">
        {props.error}
      </span>
    </div>
  );
};

export default TextInput;
