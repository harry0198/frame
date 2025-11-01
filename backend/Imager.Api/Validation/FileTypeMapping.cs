namespace Imager.Api.Validation;

public static class FileTypeMapping
{
    public static SupportedFileType? ToSupportedFileType(this string extension)
        => extension.ToLowerInvariant() switch
        {
            ".jpg" => SupportedFileType.Jpeg,
            ".jpeg" => SupportedFileType.Jpeg,
            ".png" => SupportedFileType.Png,
            ".heic" => SupportedFileType.Heic,
            _ => null
        };
}