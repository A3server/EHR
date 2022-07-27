import React from 'react'
import 'regenerator-runtime/runtime'
import './global.css'

import { ThemeProvider } from '@zendeskgarden/react-theming';
import MainPage from "./screens/MainPage";

export default function App() {
  
  return (
    <ThemeProvider>
      <MainPage/>
    </ThemeProvider>
  );

}
