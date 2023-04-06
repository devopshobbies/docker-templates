FROM mcr.microsoft.com/dotnet/sdk:6.0 AS build

WORKDIR /source

COPY . .

RUN dotnet restore && \
    dotnet publish -c release -o out

FROM mcr.microsoft.com/dotnet/aspnet:6.0 as runtime

WORKDIR /app

COPY --from=build /source/out ./

ENV PORT 80

EXPOSE 80

ENTRYPOINT ["dotnet", "dotnetcoresample.dll"]
