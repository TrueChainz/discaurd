import React from "react";

type TDisplayUserProps = {
  userId: string;
};
export type TDisplayUser = {
  selected: "DisplayUser";
  data: TDisplayUserProps;
};

const DisplayUserModal = () => {
  return <div>DisplayUserModal</div>;
};

export default DisplayUserModal;
