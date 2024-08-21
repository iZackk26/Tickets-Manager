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
        // Lee el archivo JSON
        string jsonString = File.ReadAllText("./BuyerList.json");

        // Deserializa el JSON en una lista de Client
        List<Buyer> clients = JsonSerializer.Deserialize<List<Buyer>>(jsonString);
        
        // Procesa cada cliente de manera asíncrona
        List<Task> tasks = new List<Task>();
        foreach (var client in clients)
        {
            tasks.Add(SendRequestAsync(client));
        }

        // Espera a que todas las tareas finalicen
        await Task.WhenAll(tasks);

        Console.WriteLine("Todas las solicitudes fueron procesadas.");
        
    }
    static async Task SendRequestAsync(Buyer client)
    {
        using TcpClient tcpClient = new TcpClient();
        try
        {
            // Conecta al servidor Rust
            await tcpClient.ConnectAsync("127.0.0.1", 7878); // Reemplaza con la IP y puerto adecuados ;;

            using NetworkStream stream = tcpClient.GetStream();

            // Prepara el mensaje a enviar
            var message = JsonSerializer.Serialize(client);
            var data = Encoding.UTF8.GetBytes(message);

            // Envía los datos
            await stream.WriteAsync(data, 0, data.Length);

            // Lee la respuesta del servidor
            var buffer = new byte[1024];
            var bytesRead = await stream.ReadAsync(buffer, 0, buffer.Length);

            var response = Encoding.UTF8.GetString(buffer, 0, bytesRead);
            Console.WriteLine($"Respuesta del servidor: {response}");
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Error al procesar cliente {client.section_type}: {ex.Message}");
        }
    }
}