FROM ghcr.io/siemens/kas/kas:latest
WORKDIR /workspace
COPY kas/ /workspace/kas/
COPY meta-lunnaos/ /workspace/meta-lunnaos/
CMD ["kas","build","/workspace/kas/lunnaos.yml"]