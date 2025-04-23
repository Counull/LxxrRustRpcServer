# 创建输出目录（如果不存在）



# 遍历当前目录及其子目录下的所有 .proto 文件
Get-ChildItem -Path "." -Filter "*.proto" -Recurse | ForEach-Object {
    $protoFile = $_.FullName
    $protoDir = $_.DirectoryName

  
    # 生成 C# 文件
    &protoc -I"$protoDir" --csharp_out="csharp_out" --grpc_out="csharp_out" --plugin=protoc-gen-grpc="C:\DevEnv\Tools\Protobuf\bin\dotnet-grpc.exe" "$protoFile"
}