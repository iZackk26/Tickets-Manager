import DinamicPluginLoader from "../DinamicPluginLoader";
import SeatType from "../types/Seat";
import BaseComponent from "../classes/BaseComponent";
import AxiosService from "../classes/AxiosService";
import ROUTES from "../constants/routes";

interface Props {}

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
    try {
      // Los datos que quieres enviar al backend
      const data = this.state.seats;

      // Realizar la solicitud POST al backend
      const response = await AxiosService.getInstance().post(
        ROUTES.modifySeats,
        data
      );

      // Puedes manejar la respuesta según lo que devuelva el backend
      console.log("Respuesta del backend:", response.data);
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
