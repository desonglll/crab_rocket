import axios from "axios";

export const fetch_user_by_id = async (id: number) => {
  try {
    const response = await axios.get(`user/${id}`);
    return response.data.data;
  } catch (error) {
    console.log(error);
  }
};
