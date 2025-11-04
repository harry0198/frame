using System.Diagnostics;
using Imager.Api.Common;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Options;

namespace Imager.Api.Endpoints;

internal static class UpdateInkyEndpoint
{
    public static void MapUpdateInkyEndpoint(this WebApplication app)
    {
        app.MapPost("/inky", async ([FromBody] string filePath, IOptions<ImagerOptions> imagerOptions, CancellationToken cancellation) =>
        {
            var directory = Path.GetDirectoryName(filePath);
            if (directory is null || !directory.Equals(imagerOptions.Value.Directory) || !File.Exists(filePath))
            {
                return Results.NotFound();
            }

            var psi = new ProcessStartInfo
            {
                FileName = imagerOptions.Value.InkyExecutable,
                Arguments = filePath,
                UseShellExecute = false,
                RedirectStandardOutput = true,
                RedirectStandardError = true,
            };

            using var timeoutCts = CancellationTokenSource.CreateLinkedTokenSource(cancellation);
            timeoutCts.CancelAfter(imagerOptions.Value.InkyTimeout);

            using var proc = Process.Start(psi);
            if (proc is null)
            {
                return Results.Problem("Failed to update inky display");
            }
            await proc.WaitForExitAsync(cancellation);
            
            return proc.ExitCode == 0
                ? Results.Ok()
                : Results.Problem("Failed to update inky display");
        });
    }
}