FROM python:3.9-buster as builder
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/install_rust.sh && sh /tmp/install_rust.sh -y && rm /tmp/install_rust.sh
WORKDIR /opt/app
COPY setup.py /opt/app/setup.py
COPY requirements.txt /opt/app/requirements.txt
COPY Cargo.toml /opt/app/Cargo.toml
COPY factor /opt/app/factor
COPY src /opt/app/src
RUN bash -c "source $HOME/.cargo/env && pip install -r requirements.txt && python setup.py install"

FROM python:3.9-slim-buster as runner
COPY --from=builder /usr/local/lib/python3.9/site-packages /usr/local/lib/python3.9/site-packages
CMD python
