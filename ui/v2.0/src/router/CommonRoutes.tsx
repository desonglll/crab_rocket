import {BrowserRouter, Route, Routes} from "react-router-dom";
import {Dashboard} from "../pages/Dashboard.tsx";

export const CommonRoutes = () => {
    return (
        <>
            <BrowserRouter>
                <Routes>
                    <Route path={"/"} element={<Dashboard/>}/>
                </Routes>
            </BrowserRouter>
        </>
    );
};