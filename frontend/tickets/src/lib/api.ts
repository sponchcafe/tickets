interface TicketData {
    title: string,
    description: string,
}

export default async function get_all_tickets(): Promise<TicketData[]> {
    return await fetch("http://localhost:8000/tickets")
        .then((response) => {
            console.log("STR: ", response.text);
            return response.json()
        })
        .then((data) => {
            return data;
        }).catch((err) => console.log("ERROR: "+err));
}