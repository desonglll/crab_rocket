import {Route, BrowserRouter, Routes} from "react-router-dom";
import {Home} from "./pages/Home/Home.tsx";

function App() {
    return (
        <>
            <BrowserRouter>
                <Routes>
                    <Route path={"/*"} element={<Home/>}/>
                </Routes>
            </BrowserRouter>
        </>
    );
}

export default App;
