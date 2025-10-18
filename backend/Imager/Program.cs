using Imager.Common;
using Imager.Endpoints;
using Microsoft.Extensions.FileProviders;
using Microsoft.Extensions.Options;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
// Learn more about configuring OpenAPI at https://aka.ms/aspnet/openapi
builder.Services.AddOpenApi();
builder.Services.Configure<ImagerOptions>(builder.Configuration.GetSection(nameof(ImagerOptions)));

builder.Services.AddCors(opts =>
{
    opts.AddPolicy("AllowAll", policy =>
    {
        policy
            .AllowAnyOrigin()
            .AllowAnyHeader()
            .AllowAnyMethod();
    });
});

var app = builder.Build();

app.UseCors("AllowAll");

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.MapOpenApi();
}

app.UseHttpsRedirection();

var options = app.Services.GetRequiredService<IOptions<ImagerOptions>>().Value;
var imagesDir = Path.GetFullPath(options.Directory ?? throw new InvalidOperationException("Directory not configured."));
Directory.CreateDirectory(imagesDir);

app.UseStaticFiles(new StaticFileOptions
{
    FileProvider = new PhysicalFileProvider(options.Directory),
    RequestPath = "/static/images"
});
app.MapGetImagesEndpoint();
app.MapUploadImageEndpoint();
app.MapDeleteImageEndpoint();
app.Run();