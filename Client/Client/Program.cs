// See https://aka.ms/new-console-template for more information
using System;
using System.Collections.Generic;
using System.IO;
using System.Net.Sockets;
using System.Text;
using System.Text.Json;
using System.Threading.Tasks;
using Client.Buyer;
class Program
{
    static async Task Main(string[] args)
    {
        // Load the JSON file
        string jsonString = File.ReadAllText("./BuyerList.json");

        // Deserialize the JSON string to a list of Buyer objects
        List<Buyer> clients = JsonSerializer.Deserialize<List<Buyer>>(jsonString);
        
        if (clients != null && clients.Count > 0)
        {
            // Select a random Buyer
            Random random = new Random();
            Buyer randomBuyer = clients[random.Next(clients.Count)];
            
            // Show the selected Buyer
            Console.WriteLine($"Section Type: {randomBuyer.section_type}");
            Console.WriteLine($"Quantity: {randomBuyer.quantity}");
            Console.WriteLine($"Response Time: {randomBuyer.response_time}");
            Console.WriteLine($"Response: {randomBuyer.response}");
            await SendRequestAsync(randomBuyer);
        }
        else
        {
            Console.WriteLine("No se encontraron clientes en la lista.");
            return;
        }
        
    }
    static async Task SendRequestAsync(Buyer client)
    {
        using TcpClient tcpClient = new TcpClient();
        try
        {
            // Conecta al servidor Rust
            await tcpClient.ConnectAsync("127.0.0.1", 8080); // Reemplaza con la IP y puerto adecuados
            using NetworkStream stream = tcpClient.GetStream();

            // Parte 1: Enviar section_type y quantity
            var partialClient = new { client.section_type, client.quantity };
            var partialMessage = JsonSerializer.Serialize(partialClient);
            var data = Encoding.UTF8.GetBytes(partialMessage);

            // Envía los datos
            await stream.WriteAsync(data, 0, data.Length);

            // Lee la respuesta del servidor (simulando un tiempo de espera)
            var buffer = new byte[1024];
            var bytesRead = await stream.ReadAsync(buffer, 0, buffer.Length);
            var response = Encoding.UTF8.GetString(buffer, 0, bytesRead);
            Console.WriteLine($"Respuesta del servidor: {response}");

            // Parte 2: Esperar el tiempo de respuesta y enviar el response
            await Task.Delay(client.response_time); // Simula el tiempo de espera

            var finalMessage = JsonSerializer.Serialize(new { client.response });
            data = Encoding.UTF8.GetBytes(finalMessage);
            await stream.WriteAsync(data, 0, data.Length);

            Console.WriteLine("Response enviado al servidor.");
            
            // Leer la confirmación final del servidor antes de cerrar
            bytesRead = await stream.ReadAsync(buffer, 0, buffer.Length);
            var finalResponse = Encoding.UTF8.GetString(buffer, 0, bytesRead);
            Console.WriteLine($"Confirmación final del servidor: {finalResponse}");

        }
        catch (Exception ex)
        {
            Console.WriteLine($"Error al procesar cliente {client.section_type}: {ex.Message}");
        }
    }
}