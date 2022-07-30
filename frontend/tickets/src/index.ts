
import get_all_tickets from "./lib/api";
import type {TicketData} from "./lib/api";
import "./styles/app"
console.log("test")

get_all_tickets().then((t: TicketData[]) => console.log(t));

/*
import App from './components/App.svelte'
const app = new App({
  target: document.getElementById('app')
})
export default app
*/
