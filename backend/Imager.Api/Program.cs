using Imager.Api.Common;
using Imager.Api.Endpoints;
using Microsoft.Extensions.FileProviders;
using Microsoft.Extensions.Options;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
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

builder.Services.ConfigureHttpJsonOptions(options =>
{
    options.SerializerOptions.TypeInfoResolverChain.Insert(0, JsonContext.Default);
});

builder.Host.UseSystemd();

var app = builder.Build();

app.UseCors("AllowAll");

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