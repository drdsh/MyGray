FROM rust:alpine

#git is not for github, just for local git without push, git creds do not come into this container
RUN apk add --no-cache git ca-certificates shadow bash ripgrep libgcc libstdc++ curl

#create a user account for me and possibly an AI agent
RUN groupadd -g 1000 devuser
RUN useradd -u 1000 -g 1000 -m devuser
RUN mkdir -p /home/devuser/.claude

#These two lines were for claude code who did the structure and the original draft
#for me. 
#COPY settings.json /home/devuser/.claude/settings.json
#RUN chown -R devuser:devuser /home/devuser

#create Workdir (only a mountpoint typically)
WORKDIR /workdir
USER devuser

#These two lines were for claude code who did the structure and the original draft
#for me. 
#RUN curl -fsSL https://claude.ai/install.sh | bash
#ENV PATH="$PATH:/home/devuser/.local/bin"

#As soon as the application runs and is testable via its API or http 
#we can start with AIR
CMD ["/bin/sh"]


