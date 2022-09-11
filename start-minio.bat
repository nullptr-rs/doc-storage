docker run --name MinIO -p "9000:9000" -p "9001:9001" -e "MINIO_ROOT_USER=doc-storage-user" -e "MINIO_ROOT_PASSWORD=doc-storage-password" -e "MINIO_COMPRESSION_ENABLE=on" -v MinIO:/data minio/minio server /data --console-address ":9001"