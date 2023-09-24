#include <arpa/inet.h>
#include <assert.h>
#include <errno.h>
#include <netdb.h>
#include <netinet/in.h>
#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <sys/wait.h>
#include <unistd.h>

#define PORT "8080"
#define BACKLOG -1
#define BUF_SIZE 1024

static volatile int sockfd = 0;

void int_handler(int dummy) {
    // 0 = let sends finish, receives are disallowed
    shutdown(sockfd, 0);
}

int main(void) {
    struct sockaddr_storage their_addr;
    socklen_t addr_size;
    struct addrinfo hints, *res;
    char buf[BUF_SIZE] = {0};
    int status, new_fd, bytes_read, bytes_sent;

    memset(&hints, 0, sizeof(hints));
    hints.ai_family = AF_UNSPEC; // use IPv4 or IPv6
    hints.ai_socktype = SOCK_STREAM; // socket type = TCP
    hints.ai_flags = AI_PASSIVE; // fill in my IP for me

    if ((status = getaddrinfo(NULL, PORT, &hints, &res)) != 0) {
        fprintf(stderr, "getaddrinfo: %s\n", gai_strerror(status));
        return 2;
    }

    if ((sockfd = socket(res->ai_family, res->ai_socktype, res->ai_protocol)) < 0) {
        perror("Socket creation failed");
        return 1;
    }

    if ((status = bind(sockfd, res->ai_addr, res->ai_addrlen)) != 0) {
        perror("bind failed");
        return 1;
    }

    freeaddrinfo(res);

    if ((status = listen(sockfd, BACKLOG)) != 0) {
        perror("listen failed");
        return 1;
    }

    signal(SIGINT, int_handler);

    addr_size = sizeof(their_addr);
    while (true) {
        if ((new_fd = accept(sockfd, (struct sockaddr *) &their_addr, &addr_size)) < 0) {
            if (errno == EBADF || errno == EINVAL) {
                // shutdown happened via signal
                return 0;
            }
            perror("accept failed");
            return 1;
        }

        while (true) {
            bytes_read = recv(new_fd, buf, BUF_SIZE, 0);
            // client closed connection
            if (bytes_read == 0) {
                // go to next client
                goto close_client;
            }

            if (bytes_read < 0) {
                perror("recv failed");
                return 1;
            }

            // TODO: loop over sent data, it may have been split
            if ((bytes_sent = send(new_fd, buf, bytes_read, 0)) < 0) {
                perror("send failed");
                return 1;
            }

            // seems safe to assume we've finished our work
            if (bytes_read == bytes_sent && bytes_sent < BUF_SIZE) {
                goto close_client;
            }
        }

close_client:
        close(new_fd);
    }

    return 0;
}
