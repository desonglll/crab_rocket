## Configuration of `Nginx`

```conf
server{
       listen 80;
       root /home/mike/www;
       index index.html;
       location / {
               try_files $uri $uri/ =404;

       }
}
server{
       listen 8080;
       root /home/mike/crab_rocket/ui/v0.1/dist;
       index index.html;
       location /{
               try_files $uri /index.html =404;
       }
}
```

## Production build
```shell
# npm run build -- --mode production
```