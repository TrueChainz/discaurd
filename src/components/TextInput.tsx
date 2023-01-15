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

const TextInput = (props: TextInputProps) => {
  return (
    <div className={props.containerClass}>
      {props.label && (
        <label htmlFor={props.name} className="text-sm">
          {props.label} {props.required && <span>*</span>}
        </label>
      )}
      <input
        className="rounded-md bg-slate-200 px-2 h-8"
        // ref={props.ref}
        id={props.name}
        name={props.name}
        value={props.value || ""}
        aria-invalid={!!props.error}
        aria-errormessage={`${props.name}-error`}
        required={props.required}
        onBlur={props.onBlur}
        onChange={props.onChange}
        type={props.type}
      />
      <div className="mx-auto text-center my-2">
        {props.error && (
          <span className="text-red-700 h-24 w-fit mx-auto break-words leading-3 text-center text-sm">
            {props.error}
          </span>
        )}
      </div>
    </div>
  );
};

export default TextInput;
