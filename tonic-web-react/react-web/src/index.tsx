import React from 'react'
import ReactDOM from 'react-dom/client'
import './index.css'
import {BrowserRouter, Routes, Route} from "react-router-dom"

import Page from "./pages/Page"
import NoPage from "./pages/NoPage"

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
)
root.render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route index element={<Page name="home" />} />
        <Route path="impressum" element={<Page name="impressum" />} />
        <Route path="*" element={<NoPage />} />
      </Routes>
    </BrowserRouter>
  </React.StrictMode>
)

