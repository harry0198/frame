using System.Text.Json.Serialization;
using Imager.Api.Responses;

namespace Imager.Api.Common;

[JsonSerializable(typeof(ImageUploadResponse))]
[JsonSerializable(typeof(ImageResponse))]
public partial class JsonContext : JsonSerializerContext { }