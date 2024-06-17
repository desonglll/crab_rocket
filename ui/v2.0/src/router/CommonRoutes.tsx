import {BrowserRouter, Route} from "react-router-dom";
import {Dashboard} from "../pages/Dashboard.tsx";

export const Routes = () => {
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