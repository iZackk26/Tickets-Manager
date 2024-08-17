using System;
using System.Net.Sockets;
using System.Text;

namespace Client
{
    internal class Program
    {
        public static void Main(string[] args)
        {
            try
            {
                // Conectar al servidor en la IP y puerto especificados
                using (TcpClient client = new TcpClient("127.0.0.1", 7878))
                {
                    // Obtener el stream de datos
                    NetworkStream stream = client.GetStream();

                    // Buffer para almacenar los datos recibidos
                    byte[] buffer = new byte[512];

                    // Leer datos del stream
                    int bytesRead = stream.Read(buffer, 0, buffer.Length);

                    // Convertir los bytes leídos a string
                    string message = Encoding.UTF8.GetString(buffer, 0, bytesRead);
                    Console.WriteLine("Mensaje del servidor: " + message);
                }
            }
            catch (Exception e)
            {
                Console.WriteLine("Error: " + e.Message);
            }
        }
    }
    
}

