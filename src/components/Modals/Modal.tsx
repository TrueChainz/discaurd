import React from "react";
import { z } from "zod";
// import { TProps } from "../../store/modalStore";
import AddFriendModal from "./AddFriendModal";

function ReturnModal(key: string, data: any) {
  // data.DisplayUser.
  const modals = {
    AddFriends: <AddFriendModal {...data} />,
  };

  return modals[key] ?? null;
}

const Modal = () => {
  return <div>Modal</div>;
};

export default Modal;
