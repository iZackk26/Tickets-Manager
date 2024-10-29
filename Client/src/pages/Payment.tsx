import DinamicPluginLoader from "../DinamicPluginLoader"
import SeatType from "../types/Seat";
import BaseComponent from "../classes/BaseComponent";

interface Props {}

interface State {
  seats: SeatType[];
}

class Payment extends BaseComponent<Props, State> {
  constructor(props: Props) {
    super(props);
    // Recupera los asientos seleccionados desde sessionStorage
    const seats = JSON.parse(sessionStorage.getItem('selectedSeats') || '[]');
    this.state = {
      seats: seats,
    };
  }

  render() {
    return (
      <>
        <div className="space-y-5 w-1/2">
          <DinamicPluginLoader />
          {/* Ahora puedes utilizar this.state.seats */}
          {/* Por ejemplo, mostrar los asientos seleccionados */}
          {this.state.seats.map((seat) => (
            <div key={`${seat.row}-${seat.number}`}>
              Asiento: {seat.row}-{seat.number}, Visibilidad: {seat.visibility}
            </div>
          ))}
        </div>
      </>
    );
  }
}

export default Payment;
