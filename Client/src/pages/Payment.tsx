import DinamicPluginLoader from "../DinamicPluginLoader";
import SeatType from "../types/Seat";
import BaseComponent from "../classes/BaseComponent";
import AxiosService from "../classes/AxiosService";
import ROUTES from "../constants/routes";

interface Props { }

interface State {
  seats: SeatType[];
}

class Payment extends BaseComponent<Props, State> {
  constructor(props: Props) {
    super(props);
    const seats = JSON.parse(sessionStorage.getItem('selectedSeats') || '[]');
    this.state = {
      seats: seats,
    };
  }

  handlePayClick = async () => {
    const isRejected = Math.random() > 0.5;

    if (isRejected) {
      alert("Pago rechazado");
      return;
    }

    try {
      const data = this.state.seats;
      const response = await AxiosService.getInstance().post(
        ROUTES.modifySeats,
        data
      );

      console.log("Respuesta del backend:", response.data);
      alert("Pago aceptado");

      window.location.href = "/";
    } catch (error) {
      console.error("Error al modificar los asientos:", error);
    }
  };

  render() {
    return (
      <>
        <div className="flex flex-col items-center justify-center space-y-5 w-1/2">
          <DinamicPluginLoader />
          <button
            className="bg-black text-white py-3 px-10 rounded-lg mt-4"
            onClick={this.handlePayClick}
          >
            Pay
          </button>
        </div>
      </>
    );
  }
}

export default Payment;
