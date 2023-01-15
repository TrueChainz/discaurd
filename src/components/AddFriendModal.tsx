import { Dispatch, SetStateAction } from "react";
import ReactDOM from "react-dom";
import { useAutoAnimate } from "@formkit/auto-animate/react";

interface Props {
  setModalActive: Dispatch<SetStateAction<boolean>>;
}

const AddFriendModal = ({ setModalActive }) => {
  const element = (
    <div className="modal pointer-events-auto visible opacity-100">
      <div className="modal-box flex flex-col items-center">
        <h3 className="text-lg font-bold">Add Friend</h3>
        <input
          type="text"
          placeholder="Type here"
          className="input-bordered input w-full max-w-xs"
        />
        <div className="modal-action">
          <button
            className="btn-ghost btn normal-case"
            onClick={() => setModalActive(false)}
          >
            Back
          </button>

          <button className="btn-outline btn-accent btn font-bold normal-case">
            Send Friend Request
          </button>
        </div>
      </div>
    </div>
  );
  // return ReactDOM.createPortal(element, document.getElementById("modal"));
  return element;
};

export default AddFriendModal;
