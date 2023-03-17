/**
 * \file main.js
 * 
 * \brief This file is the main entry of the frontend, it instanciates the main
 *        component of Svelte
 * 
 * \author Mathieu Dique
 */

import './app.css'
import App from './App.svelte'

const app = new App({
  target: document.getElementById('app')
})

export default app
