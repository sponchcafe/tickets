import get_all_tickets from "./lib/api";
import App from './components/App.svelte';
import "./styles/app";
import 'bootstrap';

console.log("test")
get_all_tickets().then((t) => console.log(t));

const app = new App({
  target: document.getElementById('app')
})
export default app
